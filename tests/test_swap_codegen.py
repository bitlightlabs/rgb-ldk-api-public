import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import unittest


ROOT = Path(__file__).resolve().parents[1]


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, ROOT / path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


codegen = load("api_codegen_tests", "scripts/rgb-ldk-api.py")
sync = load("api_sync_tests", "scripts/rgbldk_api_sync.py")


class SwapCodegenTests(unittest.TestCase):
    def test_external_asset_enum_has_real_serde_shape(self):
        _, enums = codegen.parse_rust_dtos([
            "pub enum LegAsset {",
            "    Btc,",
            "    Rgb { asset_id: Vec<u8> },",
            "}",
        ])
        schema = codegen.schema_for_enum(enums[0])
        self.assertEqual(schema["oneOf"][0], {"type": "string", "const": "Btc"})
        self.assertEqual(schema["oneOf"][1]["required"], ["Rgb"])
        self.assertEqual(
            schema["oneOf"][1]["properties"]["Rgb"]["properties"]["asset_id"]["type"], "array"
        )
        types = codegen.render_ts_types([], enums)
        self.assertIn('"Rgb": {', types)
        self.assertNotIn("unknown", types)

    def test_untagged_status_remains_unwrapped(self):
        _, enums = codegen.parse_rust_dtos([
            "#[serde(untagged)]",
            "pub enum Status {",
            "    Locked(LockedStatusDto),",
            "    Running(StatusDto),",
            "}",
        ])
        schema = codegen.schema_for_enum(enums[0])
        self.assertEqual(schema["oneOf"][0], {"$ref": "#/components/schemas/LockedStatusDto"})

    def test_optional_and_vector_decimals_preserve_precision_and_null(self):
        structs, _ = codegen.parse_rust_dtos([
            "pub struct Amounts {",
            '#[serde(with = "decimal_option")]',
            "pub output: Option<u64>,",
            '#[serde(with = "decimal_vec")]',
            "pub channels: Vec<u64>,",
            "}",
        ])
        types = codegen.render_ts_types(structs, [])
        self.assertIn("output: U64String | null", types)
        self.assertIn("channels: U64String[]", types)
        schema = codegen.schema_for_struct(structs[0])
        self.assertIn("output", schema["required"])
        self.assertEqual(schema["properties"]["output"]["anyOf"][1], {"type": "null"})
        self.assertEqual(schema["properties"]["channels"]["items"]["type"], "string")

    def test_extraction_excludes_domain_behavior_and_retains_codecs(self):
        source = '''#[derive(Serialize, Deserialize)]
pub struct Quote {
    #[serde(with = "decimal")]
    pub amount: u64,
}
impl Quote {
    fn execute(&self) { domain::send(); }
}
#[derive(Debug)]
pub enum PlanError {
    Shape,
}
// Decimal strings preserve SCIDs and amounts.
mod decimal {}
'''
        mirrored = sync.SwapPlansPass._wire_source(source)
        self.assertIn("pub struct Quote", mirrored)
        self.assertIn("mod decimal {}", mirrored)
        self.assertIn("utoipa::ToSchema", mirrored)
        self.assertNotIn("domain::", mirrored)
        self.assertNotIn("PlanError", mirrored)

        with self.assertRaisesRegex(RuntimeError, "decimal codecs"):
            sync.SwapPlansPass._wire_source(source.split("// Decimal strings")[0])

    def test_enriched_schema_resolves_full_execution_graph(self):
        document = json.loads((ROOT / "generated/spec/openapi.json").read_text())
        schemas = document["components"]["schemas"]
        self.assertEqual(
            schemas["SwapInfoDto"]["properties"]["execution"],
            {
                "$ref": "#/components/schemas/SwapExecution",
                "description": schemas["SwapInfoDto"]["properties"]["execution"]["description"],
            },
        )
        self.assertEqual(
            schemas["SwapCreateMultihopOfferRequest"]["properties"]["policy_hints"]["items"],
            {"$ref": "#/components/schemas/DirectionalPolicy"},
        )
        self.assertEqual(schemas["ExecutionPlan"]["properties"]["version"]["type"], "string")
        self.assertEqual(schemas["LegConstraints"]["properties"]["required_channels"]["items"]["type"],
                         "string")

        def visit(value):
            if isinstance(value, dict):
                ref = value.get("$ref")

                if ref and ref.startswith("#/components/schemas/"):
                    self.assertIn(ref.rsplit("/", 1)[1], schemas)

                for child in value.values():
                    visit(child)
            elif isinstance(value, list):
                for child in value:
                    visit(child)

        visit(document)

    def test_strip_removes_application_adapters_from_mirrored_dtos(self):
        source = sync.DtoSource(
            Path("crates/node-http/src/dto/common.rs"),
            Path("src/dto/common.rs"),
            "#[derive(Debug)]\n"
            "pub struct SweepDto {\n"
            "    pub status: String,\n"
            "}\n"
            "\n"
            "impl From<application::RgbSweepStatus> for SweepDto {\n"
            "    fn from(_: application::RgbSweepStatus) -> Self { todo!() }\n"
            "}\n",
        )
        stripped, = sync.StripDomainAdaptersPass().run((source,))
        self.assertIn("pub struct SweepDto", stripped.content)
        self.assertNotIn("application::", stripped.content)

    def test_swap_source_drops_application_view_adapter_and_trailing_tests(self):
        source = sync.DtoSource(
            Path("crates/node-http/src/dto/swap.rs"),
            Path("src/dto/swap.rs"),
            "pub struct SwapInfoDto {\n"
            "    pub status: String,\n"
            "}\n"
            "\n"
            "impl From<application::SwapView> for SwapInfoDto {\n"
            "    fn from(_: application::SwapView) -> Self { todo!() }\n"
            "}\n"
            "\n"
            "#[cfg(test)]\n"
            "mod tests {\n"
            "    fn view() { let _ = application::SwapView { execution: None }; }\n"
            "}\n",
        )
        stripped, = sync.StripDomainAdaptersPass().run((source,))
        self.assertIn("pub struct SwapInfoDto", stripped.content)
        self.assertNotIn("application::", stripped.content)

    def test_generation_is_reproducible(self):
        paths = [
            ROOT / "packages/rgbldk-node-api-types/src/generated.ts",
            ROOT / "packages/rgbldk-node-client/src/generated.ts",
            ROOT / "crates/rgbldk_http_client/src/generated.rs",
            ROOT / "generated/spec/source.json",
        ]
        before = [path.read_bytes() for path in paths]
        subprocess.run([sys.executable, str(ROOT / "scripts/rgb-ldk-api.py"), "gen"], check=True)
        self.assertEqual(before, [path.read_bytes() for path in paths])


if __name__ == "__main__":
    unittest.main()
