use prost_types::{
    uninterpreted_option::NamePart, DescriptorProto, FileDescriptorProto, MessageOptions,
    UninterpretedOption,
};

pub fn convert_file_descriptor_proto(
    file_descriptor_proto: protobuf::descriptor::FileDescriptorProto,
) -> FileDescriptorProto {
    FileDescriptorProto {
        name: file_descriptor_proto.name,
        package: file_descriptor_proto.package,
        dependency: file_descriptor_proto.dependency,
        public_dependency: file_descriptor_proto.public_dependency,
        weak_dependency: file_descriptor_proto.weak_dependency,
        message_type: file_descriptor_proto
            .message_type
            .into_iter()
            .map(|descriptor_proto| DescriptorProto {
                name: descriptor_proto.name,
                field: descriptor_proto
                    .field
                    .into_iter()
                    .map(|field_descriptor_proto| todo!())
                    .collect(),
                extension: descriptor_proto
                    .extension
                    .into_iter()
                    .map(|el| todo!())
                    .collect(),
                nested_type: descriptor_proto
                    .nested_type
                    .into_iter()
                    .map(|el| todo!())
                    .collect(),
                enum_type: descriptor_proto
                    .enum_type
                    .into_iter()
                    .map(|el| todo!())
                    .collect(),
                extension_range: descriptor_proto
                    .extension_range
                    .into_iter()
                    .map(|el| todo!())
                    .collect(),
                oneof_decl: descriptor_proto
                    .oneof_decl
                    .into_iter()
                    .map(|el| todo!())
                    .collect(),
                options: descriptor_proto
                    .options
                    .0
                    .map(|el| convert_message_options(*el)),
                reserved_range: descriptor_proto
                    .reserved_range
                    .into_iter()
                    .map(|el| todo!())
                    .collect(),
                reserved_name: descriptor_proto
                    .reserved_name
                    .into_iter()
                    .map(|el| todo!())
                    .collect(),
            })
            .collect(),
        enum_type: file_descriptor_proto
            .enum_type
            .into_iter()
            .map(|el| todo!())
            .collect(),
        service: file_descriptor_proto
            .service
            .into_iter()
            .map(|el| todo!())
            .collect(),
        extension: file_descriptor_proto
            .extension
            .into_iter()
            .map(|el| todo!())
            .collect(),
        options: todo!(),
        source_code_info: todo!(),
        syntax: file_descriptor_proto.syntax,
    }
}

fn convert_descriptor_proto(
    descriptor_proto: protobuf::descriptor::DescriptorProto,
) -> DescriptorProto {
    DescriptorProto {
        name: todo!(),
        field: todo!(),
        extension: todo!(),
        nested_type: todo!(),
        enum_type: todo!(),
        extension_range: todo!(),
        oneof_decl: todo!(),
        options: todo!(),
        reserved_range: todo!(),
        reserved_name: todo!(),
    }
}

fn convert_message_options(
    message_options: protobuf::descriptor::MessageOptions,
) -> MessageOptions {
    MessageOptions {
        message_set_wire_format: message_options.message_set_wire_format,
        no_standard_descriptor_accessor: message_options.no_standard_descriptor_accessor,
        deprecated: message_options.deprecated,
        map_entry: message_options.map_entry,
        uninterpreted_option: message_options
            .uninterpreted_option
            .into_iter()
            .map(|el| convert_interpreted_option(el))
            .collect(),
    }
}

fn convert_interpreted_option(
    interpreted_option: protobuf::descriptor::UninterpretedOption,
) -> UninterpretedOption {
    UninterpretedOption {
        name: interpreted_option
            .name
            .into_iter()
            .map(|el| convert_name_part(el))
            .collect(),
        identifier_value: interpreted_option.identifier_value,
        positive_int_value: interpreted_option.positive_int_value,
        negative_int_value: interpreted_option.negative_int_value,
        double_value: interpreted_option.double_value,
        string_value: interpreted_option.string_value,
        aggregate_value: interpreted_option.aggregate_value,
    }
}

fn convert_name_part(name_part: protobuf::descriptor::uninterpreted_option::NamePart) -> NamePart {
    NamePart {
        name_part: name_part.name_part.unwrap_or_default(),
        is_extension: name_part.is_extension.unwrap_or_default(),
    }
}
