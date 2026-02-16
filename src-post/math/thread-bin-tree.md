

이진 트리를 보면 실제 포인터보다 더 많은 널 링크가 있음을 알 수 있다

이진 트리에는 총 2n 개의 링크 중에 n + 1 개의 널 링크가 있다

스레드 이진 트리는 이런 널 링크들을 이용하는 방법이다

널 링크를 스레드라 하는 다른 노드를 가맄티는 포인터로 대치하였다.
- ptr -> leftChild 널이면 prt->leftChild 를 중위순회 전 단계에 방문한 노드를 가리킨다.
  - 이 것은 널 링크를 ptr 의 중위 선행자(inorder predecessor)에 대한 포인터로 대치하는 것이다
  - ex) I(ptr) -> leftChild 가 널이기에 I 전 단계인 D 를 가리킴
- ptr -> rightChild 가 널이면 ptr->rightChild 를 중위순회 다음 단계 포인터로 대치한다.
  - ptr 의 중위 후속자(inorder successor)에 대한 포인터로 대치하는 것이다 
  - ex) E(ptr) -> rightChild 가 널이기에 E 다음 단계인 A 를 가리킴
- H, G 와 같이 옆에 가리킬 노드가 없는 분실 쓰레드 상황에서는 root 를 가리킨다. 

```text
                       (A)
                    /   ^    \        root 
                (B)     |      (C)     ^
             /   ^  \   |     / ^ \    |
         (D)     |__(E)_|__(F)__|__(G)_|
  root  /   \    |
    ^ (H)   (I)  |
    |  ^     ^   |
    |__|     |___|
```

이 트리를 중위 순회하면 H -> D -> I -> B -> E -> A -> F -> C -> G 가 된다

노드 E 는 B 를 가리키는 선행자 스레드와 A 를 가리키는 후속자 스레드를 가지고 있다.

**이 "스레드 트리"를 사용하면 스택을 사용하지 않고 중위 순회를 수행할 수 있다**
- 현재 노드 tpr 에서 ptr->rightThread = TRUE 
  - ptr 의 중위 후속자는 스레드 정의에 의해 ptr -> rightChild 이다
- 현재 노드 tpr 에서 ptr->rightThread = FALSE
  - ptr 의 중위 후속자는 ptr 의 오른쪽 자식으로부터 시작하여 왼쪽 자식 링크를 따라 leftThread = TRUE 인 노드에 도달할 때까지 찾으면 된다
  