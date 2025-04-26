# Broker Structure


## Broker Structure

``` typescript
struct Broker {
  address: string,

  topics: Topic[],
  brokers: Broker[],
}
```

## Broker Functions

### 1. Add Topic
``` typescript
function addTopic(topic: Topic): void
```

### 2. Remove Topic
``` typescript
function removeTopic(topic: Topic): void
```

### 3. Get Topic
``` typescript
function getTopic(topicName: string): Topic | null
```

### 4. Get All Topics
``` typescript
function getAllTopics(): Topic[]
```

### 3. Add Broker
``` typescript
function addBroker(broker: Broker): void
```

### 4. Remove Broker
``` typescript
function removeBroker(broker: Broker): void
```