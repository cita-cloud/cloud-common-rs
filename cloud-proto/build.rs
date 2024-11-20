// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "tonic-build")]
    {
        println!("cargo:rerun-if-changed=protos");
        tonic_build::configure()
            .out_dir("src/proto")
            .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
            .compile_protos(
                &[
                    "blockchain.proto",
                    "common.proto",
                    "consensus.proto",
                    "network.proto",
                    "storage.proto",
                    "crypto.proto",
                    "health_check.proto",
                    "status_code.proto",
                ],
                &["protos/protos"],
            )?;
        tonic_build::configure()
            .out_dir("src/proto")
            .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
            .file_descriptor_set_path("src/reflect/controller.bin")
            .compile_protos(&["controller.proto"], &["protos/protos"])?;
        tonic_build::configure()
            .out_dir("src/proto")
            .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
            .file_descriptor_set_path("src/reflect/executor.bin")
            .compile_protos(&["executor.proto", "vm/evm.proto"], &["protos/protos"])?;
    }
    Ok(())
}
