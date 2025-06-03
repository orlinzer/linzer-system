# Connection

## Connection Structure

```typescript
struct Connection {
  brokerId: string,
  address: string,

  state: BrokerState,

  topics: Topic[],
}
```

## Connection Functions

### Constructors

1. Create Connection

```typescript
function createConnection(address: string): Connection;
```

### Destructor

1. Destroy Connection

```typescript
function destroyConnection(broker: Connection): void;
```

### Address Functions

1. Update Address

```typescript
function updateAddress(broker: Connection, newAddress: string): void;
```

1.  Get Address

```typescript
function getAddress(broker: Connection): string;
```

### Topic Functions

1. Add Topic

```typescript
function addTopic(topic: Topic): void;
```

2. Update Topic

```typescript
function updateTopic(topic: Topic): void;
```

3. Remove Topic

```typescript
function removeTopic(topic: Topic): void;
```

4. Get All Topics

```typescript
function getAllTopics(): Topic[];
```

5. Get Topics Count

```typescript
function getTopicsCount(): number;
```

6. Get Topic

```typescript
function getTopic(topicName: string): Topic | null;
```

### Connection Functions

1. Add Connection

```typescript
function addConnection(broker: Connection): void;
```

2. Update Connection

```typescript
function updateConnection(broker: Connection): void;
```

3. Remove Connection

```typescript
function removeConnection(broker: Connection): void;
```

4. Get All Connections

```typescript
function getAllConnections(): Connection[];
```

5. Get Connections Count

```typescript
function getConnectionsCount(): number;
```

6. Get Connection

```typescript
function getConnection(brokerAddress: string): Connection | null;
```
