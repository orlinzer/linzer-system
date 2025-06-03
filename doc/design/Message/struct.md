# Message

## Message Structure

```typescript
struct Trace {
  brokerId: string,
  timestamp: number,
}

struct MessageMetaData {
  id: string,
  topic: string,
  trace: Trace[],
  length: number,
}

struct Message {
  metaData: MessageMetaData,
  data: void*,
}
```

## Message Functions

### Constructors

1. Create Message

```typescript
function createMessage(
  name: string,
  config: {
    replicationFactor: number;
    PreferDataInOrder: bool;
    PreferDataLoss: bool;
  }
): Message;
```

### Destructor

1. Destroy Message

```typescript
function destroyMessage(topic: Message): void;
```

### Name Functions

1. Update Name

```typescript
function updateName(topic: Message, newName: string): void;
```

2. Get Name

```typescript
function getName(topic: Message): string;
```

### Consumer Functions

1. Add Consumer

```typescript
function addConsumer(topic: Message, consumer: Consumer): void;
```

2. Update Consumer

```typescript
function updateConsumer(topic: Message, consumer: Consumer): void;
```

3. Remove Consumer

```typescript
function removeConsumer(topic: Message, consumer: Consumer): void;
```

4. Get Consumers

```typescript
function getConsumers(topic: Message): Consumer[];
```

5. Get Consumer Count

```typescript
function getConsumerCount(topic: Message): number;
```

6. Get Consumer

```typescript
function getConsumer(topic: Message, consumerId: string): Consumer;
```

### Messages Functions

1. Add Message

```typescript
function addMessage(topic: Message, message: Message): void;
```

2. Update Message

```typescript
function updateMessage(topic: Message, message: Message): void;
```

3. Remove Message

```typescript
function removeMessage(topic: Message, message: Message): void;
```

4. Get Messages

```typescript
function getMessages(topic: Message): Message[];
```

5. Get Messages Count

```typescript
function getMessagesCount(topic: Message): number;
```

6. Get Message

```typescript
function getMessage(topic: Message, messageId: string): Message;
```
