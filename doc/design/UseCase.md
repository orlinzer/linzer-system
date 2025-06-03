# Use Case

## Publisher User Story

1. **Connection**: The publisher connects to brokers using the brokers addresses.
2. **Publish**: The publisher sends messages to a specific topic.
3. **Acknowledge**: The publisher receives an acknowledgment from the broker that the message has been received.
   3.1. The publisher receives an acknowledgment from the broker that the message has been received by the broker.
   3.2. The publisher receives an acknowledgment from the broker that the message has been received by all the consumers.
4. **Disconnect**: The publisher disconnects from the brokers.

## Consumer User Story 1

1. **Connection**: The consumer connects to brokers using the brokers addresses.
2. **Subscribe**: The consumer subscribes to a specific topic.
3. **Receive**: The consumer receives messages from the topic.
4. **Disconnect**: The consumer disconnects from the brokers.

## Consumer User Story 2

1. **Connection**: The consumer connects to brokers using the brokers addresses.
2. **Get Messages**: The consumer retrieves a specific messages from a specific topic.
3. **Disconnect**: The consumer disconnects from the brokers.
