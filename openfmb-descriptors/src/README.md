<!--
SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc

SPDX-License-Identifier: Apache-2.0
-->

# OpenFMB Descriptors

The protobuf compiler, protoc, takes protobuf idl files (.proto) and generates
a description of the files and messages as a protobuf encoded value (FileDescriptorSet)
which can be used to then generate code.

OpenFMB is well described in UML and converted to .proto message descriptors.
This rust crate contains a prebuilt, statically allocated instance of the
FileDescriptorSet using prost and protoc. This can then be used to generate code
in rust at compile time with build.rs builders.
