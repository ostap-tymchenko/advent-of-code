- Each input is a monkey
- A monkey holds:
    - Its inventory (worry items)
    - Its multiply_item_by value (new = old * multiply_item_by)
    - Its divisibility test (if divisible by X send to monkey A. else send to monkey B) 

- For each monkey 
    - A monkeys cycle (executes on each (for each) item held by that monkey) is:
        - item is multiplied (as) new = old * multiply_item_by
        - item is devided by 3
        - if item is devisible by X send to monkey A, else send to B

    - As a monkey sends an item to a new monkey, the item is added to its inventory, and in the same round, or next round if that monkey already went, the new monkey will consider it, alongside all the ones that were already in its inventory.
- The awnser is [most active monkey tally * second most active monkey tally]

```rust
struct Monkeyid (i32);

struct Divisibility_test {
    devisor: i32,
    send_to_if_true: Monkeyid,
    send_to_if_false: Monkeyid,
};

struct Monkey {
    mid: Monkeyid,
    inventory: Hashset<i32>,
    div_test: Divisibility_test,
    item_send_tally: i32
};
```
