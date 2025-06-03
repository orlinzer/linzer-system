# Topic

## Topic Structure

```typescript
struct TopicConfig {
  id: string,
  replicationFactor: number,
  PreferDataInOrder: bool,
  PreferDataLoss: bool,
}

struct TopicState {
  noOfConsumers: number,
  noOfMessages: number,
}

struct TopicMetaData {
  config: TopicConfig,
  state: TopicState,
}

struct Topic {
  metaData: TopicMetaData,

  consumers: Connection[],
  messages: Message[],
}
```

## Topic Functions

### Constructors

1. Create Topic

```typescript
function createTopic(name: string, config: TopicConfig): Topic;
```

### Destructor

1. Destroy Topic

```typescript
function destroyTopic(topic: Topic): void;
```

### Name Functions

1. Update Name

```typescript
function updateName(topic: Topic, newName: string): void;
```

2. Get Name

```typescript
function getName(topic: Topic): string;
```

### Consumer Functions

1. Add Consumer

```typescript
function addConsumer(topic: Topic, consumer: Consumer): void;
```

2. Update Consumer

```typescript
function updateConsumer(topic: Topic, consumer: Consumer): void;
```

3. Remove Consumer

```typescript
function removeConsumer(topic: Topic, consumer: Consumer): void;
```

4. Get Consumers

```typescript
function getConsumers(topic: Topic): Consumer[];
```

5. Get Consumer Count

```typescript
function getConsumerCount(topic: Topic): number;
```

6. Get Consumer

```typescript
function getConsumer(topic: Topic, consumerId: string): Consumer;
```

### Messages Functions

1. Add Message

```typescript
function addMessage(topic: Topic, message: Message): void;
```

2. Update Message

```typescript
function updateMessage(topic: Topic, message: Message): void;
```

3. Remove Message

```typescript
function removeMessage(topic: Topic, message: Message): void;
```

4. Get Messages

```typescript
function getMessages(topic: Topic): Message[];
```

5. Get Messages Count

```typescript
function getMessagesCount(topic: Topic): number;
```

6. Get Message

```typescript
function getMessage(topic: Topic, messageId: string): Message;
```
