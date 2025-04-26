# Topic Structure

``` typescript
struct Topic {
  name: string,

  consumers: Consumer[] ,
  messages: Message[],

  PreferDataInOrder: bool,
  PreferDataLoss: bool,
}
```
