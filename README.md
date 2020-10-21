# OpenFMB

An Open Field Message Bus. This library provides a set of traits for
controlling and monitoring power grid devices over a message bus.

## Code Generation

The OpenFMB specification is defined in UML and converted to potentially 
any number of message encodings. The most common is protocol buffers.

The UML generates additional information to describe an single-inheritance
message type hierarchy that cannot be directly expressed in a proto or
correctly code generated for using protoc and Rusts prost alone.

So instead a fork of prost-build and prost-types were vendored here from
prost 0.6 (and should be updated as needed) to expand on the code generation
to fit OpenFMB needs.
