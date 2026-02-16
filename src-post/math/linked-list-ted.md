normal version

```cpp
void remove_list_entry(const Node *entry) {
    Node *prev = nullptr;
    Node *walk = g_head;

    while (walk != entry) {
        prev = walk;
        walk = walk->next;
    }

    if (prev == nullptr)
        g_head = entry->next;
    else
        prev->next = entry->next;
}
```

better version

```cpp
void remove_list_entry_better(const Node *entry) {

    // indirect changes
    // head -> next -> next ...

    Node **indirect = &g_head;

    while ((*indirect) != entry) {
        indirect = &(*indirect)->next;
    }
    *indirect = entry->next;
}
```
- ted: https://youtu.be/o8NPllzkFhE?t=867