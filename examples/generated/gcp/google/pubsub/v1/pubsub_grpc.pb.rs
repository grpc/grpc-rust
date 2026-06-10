/// Generated client implementations.
pub mod publisher_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    /// The service that an application uses to manipulate topics, and to send
    /// messages to a topic.
    #[derive(Debug, Clone)]
    pub struct PublisherClient<T> {
        channel: T,
    }

    impl<T> PublisherClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        /// Creates the given topic with the given name. See the \[resource name rules\]
        /// (https://cloud.google.com/pubsub/docs/admin\#resource\_names).
        pub fn create_topic<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Topic>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::Topic> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.Publisher/CreateTopic", request)
        }

        /// Updates an existing topic. Note that certain properties of a
        /// topic are not modifiable.
        pub fn update_topic<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Topic>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::UpdateTopicRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.Publisher/UpdateTopic", request)
        }

        /// Adds one or more messages to the topic. Returns \`NOT\_FOUND\` if the topic
        /// does not exist.
        pub fn publish<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::PublishResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::PublishRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.Publisher/Publish", request)
        }

        /// Gets the configuration of a topic.
        pub fn get_topic<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Topic>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::GetTopicRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.Publisher/GetTopic", request)
        }

        /// Lists matching topics.
        pub fn list_topics<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::ListTopicsResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::ListTopicsRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.Publisher/ListTopics", request)
        }

        /// Lists the names of the attached subscriptions on this topic.
        pub fn list_topic_subscriptions<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::ListTopicSubscriptionsResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::ListTopicSubscriptionsRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.Publisher/ListTopicSubscriptions", request)
        }

        /// Lists the names of the snapshots on this topic. Snapshots are used in
        /// \[Seek\](https://cloud.google.com/pubsub/docs/replay-overview) operations,
        /// which allow you to manage message acknowledgments in bulk. That is, you can
        /// set the acknowledgment state of messages in an existing subscription to the
        /// state captured by a snapshot.
        pub fn list_topic_snapshots<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::ListTopicSnapshotsResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::ListTopicSnapshotsRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.Publisher/ListTopicSnapshots", request)
        }

        /// Deletes the topic with the given name. Returns \`NOT\_FOUND\` if the topic
        /// does not exist. After a topic is deleted, a new topic may be created with
        /// the same name; this is an entirely new topic with none of the old
        /// configuration or subscriptions. Existing subscriptions to this topic are
        /// not deleted, but their \`topic\` field is set to \`\_deleted-topic\_\`.
        pub fn delete_topic<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, ::protobuf_well_known_types::Empty>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::DeleteTopicRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.Publisher/DeleteTopic", request)
        }

        /// Detaches a subscription from this topic. All messages retained in the
        /// subscription are dropped. Subsequent \`Pull\` and \`StreamingPull\` requests
        /// will return FAILED\_PRECONDITION. If the subscription is a push
        /// subscription, pushes to the endpoint will stop.
        pub fn detach_subscription<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::DetachSubscriptionResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::DetachSubscriptionRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.Publisher/DetachSubscription", request)
        }
    }
}
/// Generated client implementations.
pub mod subscriber_client {
    use grpc::client::*;
    use grpc_protobuf::*;

    /// The service that an application uses to manipulate subscriptions and to
    /// consume messages from a subscription via the \`Pull\` method or by
    /// establishing a bi-directional stream using the \`StreamingPull\` method.
    #[derive(Debug, Clone)]
    pub struct SubscriberClient<T> {
        channel: T,
    }

    impl<T> SubscriberClient<T>
    where
        T: grpc::client::Invoke,
    {
        pub fn new(channel: T) -> Self {
            Self { channel }
        }

        /// Creates a subscription to a given topic. See the \[resource name rules\]
        /// (https://cloud.google.com/pubsub/docs/admin\#resource\_names).
        /// If the subscription already exists, returns \`ALREADY\_EXISTS\`.
        /// If the corresponding topic doesn't exist, returns \`NOT\_FOUND\`.
        ///
        /// If the name is not provided in the request, the server will assign a random
        /// name for this subscription on the same project as the topic, conforming
        /// to the \[resource name format\]
        /// (https://cloud.google.com/pubsub/docs/admin\#resource\_names). The generated
        /// name is populated in the returned Subscription object. Note that for REST
        /// API requests, you must specify a name in the request.
        pub fn create_subscription<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Subscription>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::Subscription> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.Subscriber/CreateSubscription", request)
        }

        /// Gets the configuration details of a subscription.
        pub fn get_subscription<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Subscription>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::GetSubscriptionRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.Subscriber/GetSubscription", request)
        }

        /// Updates an existing subscription. Note that certain properties of a
        /// subscription, such as its topic, are not modifiable.
        pub fn update_subscription<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Subscription>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::UpdateSubscriptionRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.Subscriber/UpdateSubscription", request)
        }

        /// Lists matching subscriptions.
        pub fn list_subscriptions<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::ListSubscriptionsResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::ListSubscriptionsRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.Subscriber/ListSubscriptions", request)
        }

        /// Deletes an existing subscription. All messages retained in the subscription
        /// are immediately dropped. Calls to \`Pull\` after deletion will return
        /// \`NOT\_FOUND\`. After a subscription is deleted, a new one may be created with
        /// the same name, but the new one has no association with the old
        /// subscription or its topic unless the same topic is specified.
        pub fn delete_subscription<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, ::protobuf_well_known_types::Empty>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::DeleteSubscriptionRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.Subscriber/DeleteSubscription", request)
        }

        /// Modifies the ack deadline for a specific message. This method is useful
        /// to indicate that more time is needed to process a message by the
        /// subscriber, or to make the message available for redelivery if the
        /// processing was interrupted. Note that this does not modify the
        /// subscription-level \`ackDeadlineSeconds\` used for subsequent messages.
        pub fn modify_ack_deadline<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, ::protobuf_well_known_types::Empty>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::ModifyAckDeadlineRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.Subscriber/ModifyAckDeadline", request)
        }

        /// Acknowledges the messages associated with the \`ack\_ids\` in the
        /// \`AcknowledgeRequest\`. The Pub/Sub system can remove the relevant messages
        /// from the subscription.
        ///
        /// Acknowledging a message whose ack deadline has expired may succeed,
        /// but such a message may be redelivered later. Acknowledging a message more
        /// than once will not result in an error.
        pub fn acknowledge<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, ::protobuf_well_known_types::Empty>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::AcknowledgeRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.Subscriber/Acknowledge", request)
        }

        /// Pulls messages from the server. The server may return \`UNAVAILABLE\` if
        /// there are too many concurrent pull requests pending for the given
        /// subscription.
        pub fn pull<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::PullResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::PullRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.Subscriber/Pull", request)
        }

        /// Establishes a stream with the server, which sends messages down to the
        /// client. The client streams acknowledgements and ack deadline modifications
        /// back to the server. The server will close the stream and return the status
        /// on any error. The server may close the stream with status \`UNAVAILABLE\` to
        /// reassign server-side resources, in which case, the client should
        /// re-establish the stream. Flow control can be achieved by configuring the
        /// underlying RPC channel.
        pub fn streaming_pull(&self) -> BidiCallBuilder<'_, &T, super::StreamingPullRequest, super::StreamingPullResponse> {
          BidiCallBuilder::new(&self.channel, "/google.pubsub.v1.Subscriber/StreamingPull")
        }

        /// Modifies the \`PushConfig\` for a specified subscription.
        ///
        /// This may be used to change a push subscription to a pull one (signified by
        /// an empty \`PushConfig\`) or vice versa, or change the endpoint URL and other
        /// attributes of a push subscription. Messages will accumulate for delivery
        /// continuously through the call regardless of changes to the \`PushConfig\`.
        pub fn modify_push_config<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, ::protobuf_well_known_types::Empty>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::ModifyPushConfigRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.Subscriber/ModifyPushConfig", request)
        }

        /// Gets the configuration details of a snapshot. Snapshots are used in
        /// \<a href="https://cloud.google.com/pubsub/docs/replay-overview"\>Seek\</a\>
        /// operations, which allow you to manage message acknowledgments in bulk. That
        /// is, you can set the acknowledgment state of messages in an existing
        /// subscription to the state captured by a snapshot.
        pub fn get_snapshot<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Snapshot>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::GetSnapshotRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.Subscriber/GetSnapshot", request)
        }

        /// Lists the existing snapshots. Snapshots are used in \[Seek\](
        /// https://cloud.google.com/pubsub/docs/replay-overview) operations, which
        /// allow you to manage message acknowledgments in bulk. That is, you can set
        /// the acknowledgment state of messages in an existing subscription to the
        /// state captured by a snapshot.
        pub fn list_snapshots<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::ListSnapshotsResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::ListSnapshotsRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.Subscriber/ListSnapshots", request)
        }

        /// Creates a snapshot from the requested subscription. Snapshots are used in
        /// \[Seek\](https://cloud.google.com/pubsub/docs/replay-overview) operations,
        /// which allow you to manage message acknowledgments in bulk. That is, you can
        /// set the acknowledgment state of messages in an existing subscription to the
        /// state captured by a snapshot.
        /// If the snapshot already exists, returns \`ALREADY\_EXISTS\`.
        /// If the requested subscription doesn't exist, returns \`NOT\_FOUND\`.
        /// If the backlog in the subscription is too old -- and the resulting snapshot
        /// would expire in less than 1 hour -- then \`FAILED\_PRECONDITION\` is returned.
        /// See also the \`Snapshot.expire\_time\` field. If the name is not provided in
        /// the request, the server will assign a random
        /// name for this snapshot on the same project as the subscription, conforming
        /// to the \[resource name format\]
        /// (https://cloud.google.com/pubsub/docs/admin\#resource\_names). The
        /// generated name is populated in the returned Snapshot object. Note that for
        /// REST API requests, you must specify a name in the request.
        pub fn create_snapshot<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Snapshot>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::CreateSnapshotRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.Subscriber/CreateSnapshot", request)
        }

        /// Updates an existing snapshot. Snapshots are used in
        /// \<a href="https://cloud.google.com/pubsub/docs/replay-overview"\>Seek\</a\>
        /// operations, which allow
        /// you to manage message acknowledgments in bulk. That is, you can set the
        /// acknowledgment state of messages in an existing subscription to the state
        /// captured by a snapshot.
        pub fn update_snapshot<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::Snapshot>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::UpdateSnapshotRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.Subscriber/UpdateSnapshot", request)
        }

        /// Removes an existing snapshot. Snapshots are used in \[Seek\]
        /// (https://cloud.google.com/pubsub/docs/replay-overview) operations, which
        /// allow you to manage message acknowledgments in bulk. That is, you can set
        /// the acknowledgment state of messages in an existing subscription to the
        /// state captured by a snapshot.
        /// When the snapshot is deleted, all messages retained in the snapshot
        /// are immediately dropped. After a snapshot is deleted, a new one may be
        /// created with the same name, but the new one has no association with the old
        /// snapshot or its subscription, unless the same subscription is specified.
        pub fn delete_snapshot<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, ::protobuf_well_known_types::Empty>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::DeleteSnapshotRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.Subscriber/DeleteSnapshot", request)
        }

        /// Seeks an existing subscription to a point in time or to a given snapshot,
        /// whichever is provided in the request. Snapshots are used in \[Seek\]
        /// (https://cloud.google.com/pubsub/docs/replay-overview) operations, which
        /// allow you to manage message acknowledgments in bulk. That is, you can set
        /// the acknowledgment state of messages in an existing subscription to the
        /// state captured by a snapshot. Note that both the subscription and the
        /// snapshot must be on the same topic.
        pub fn seek<ReqMsgView>(
            &self,
            request: ReqMsgView,
        ) -> UnaryCallBuilder<'_, &T, ReqMsgView, super::SeekResponse>
        where
          ReqMsgView: protobuf::AsView<Proxied = super::SeekRequest> + Send + Sync {
          UnaryCallBuilder::new(&self.channel, "/google.pubsub.v1.Subscriber/Seek", request)
        }
    }
}
