/// Generated client implementations.
pub mod schema_service_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    #[derive(Debug, Clone)]
    pub struct SchemaServiceClient<T> {
        channel: T,
    }

    impl<T> SchemaServiceClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        /// Creates a schema.
        pub fn create_schema<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Schema>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::CreateSchemaRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.SchemaService/CreateSchema", request)
        }

        /// Gets a schema.
        pub fn get_schema<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Schema>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::GetSchemaRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.SchemaService/GetSchema", request)
        }

        /// Lists schemas in a project.
        pub fn list_schemas<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::ListSchemasResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::ListSchemasRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.SchemaService/ListSchemas", request)
        }

        /// Deletes a schema.
        pub fn delete_schema<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, ::protobuf_well_known_types::Empty>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::DeleteSchemaRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.SchemaService/DeleteSchema", request)
        }

        /// Validates a schema.
        pub fn validate_schema<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::ValidateSchemaResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::ValidateSchemaRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.SchemaService/ValidateSchema", request)
        }

        /// Validates a message against a schema.
        pub fn validate_message<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::ValidateMessageResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::ValidateMessageRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.SchemaService/ValidateMessage", request)
        }
    }
}
