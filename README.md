<!--
SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc

SPDX-License-Identifier: Apache-2.0
-->

# openfmb.rs

An Open Field Message Bus crate for Rust.

## Code Generation

The OpenFMB specification is defined in UML and converted to potentially 
any number of message encodings. The most common is protocol buffers.

The UML generates additional information to describe a single-inheritance
message type hierarchy that cannot be directly expressed in a proto or
correctly code generated for using protoc and Rusts prost alone.

So instead a fork of prost-build and prost-types were vendored here from
prost 0.6 (and should be updated as needed) to expand on the code generation
to fit OpenFMB needs.

## Examples

Running examples requires a few environment variables be set, depending on the
example.

The switch_device and switch_client examples require a NATS_URL 
and SWITCH_MRID environment variables be set

```sh 
RUST_BACKTRACE=1 RUST_LOG=debug NATS_URL="localhost:4222" SWITCH_MRID="6e595d68-67b4-434c-8c26-736104cc14fe" cargo run --example=switch_device --features="nats-sync client"
```

```sh 
RUST_BACKTRACE=1 RUST_LOG=debug NATS_URL="localhost:4222" SWITCH_MRID="6e595d68-67b4-434c-8c26-736104cc14fe" cargo run --example=switch_client --features="nats-sync client zenoh"
```

## Commercial licenses and support

Support and commercial license options are provided by Open Energy Solutions Inc. Please contact info@openenergysolutions.com for more details.

## Contributing

If you want to contribute to the improvement and development of the library please send me comments, feature requests, bug reports, or patches. For more than trivial contributions I require you to sign a Contributor License Agreement. Please contact info@openenergysolutions.com.

Please don't send pull requests before signing the Contributor License Agreement! Such pull requests may be silently ignored.
