# Broker

## Broker Structure

```typescript
struct BrokerConfig {
  id: string,
  port: number,
}

struct BrokerState {
  noOfTopics: number,
  noOfConnections: number,
}

struct BrokerMetaData {
  config: BrokerConfig,
  state: BrokerState,
}

struct Broker {
  metaData: BrokerMetaData,

  topics: Topic[],
  connections: Connection[],
}
```

## Broker Functions

### Constructors

1. Create Broker

```typescript
function createBroker(address: string): Broker;
```

### Destructor

1. Destroy Broker

```typescript
function destroyBroker(broker: Broker): void;
```

### Address Functions

1. Update Address

```typescript
function updateAddress(broker: Broker, newAddress: string): void;
```

1.  Get Address

```typescript
function getAddress(broker: Broker): string;
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
function addConnection(connection: Connection): void;
```

2. Update Connection

```typescript
function updateConnection(connection: Connection): void;
```

3. Remove Connection

```typescript
function removeConnection(connection: Connection): void;
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
function getConnection(connectionAddress: string): Connection | null;
```
