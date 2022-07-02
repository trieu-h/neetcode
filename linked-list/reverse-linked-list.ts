interface ListNode<T> {
  val: T;
  next: ListNode<T> | null
}

class ListNode<T> {
    val: T
    next: ListNode<T> | null
    constructor(val?: T, next?: ListNode<T> | null) {
        this.val = val;
        this.next = next;
    }
}

function reverseList<T>(head: ListNode<T> | null): ListNode<T> | null {
  let cur = head;
  let prev = null;
  while (cur) {
    const nextNode = cur.next;
    cur.next = prev;
    prev = cur;
    cur = nextNode;
  }
  return prev;
};

function printList<T>(head: ListNode<T>) {
  let cur = head;
  while(cur) {
    console.log(cur.val);
    cur = cur.next;
  }
}

function main() {
  const headNode = new ListNode(1, new ListNode(2, null));
  const reverse = reverseList(headNode);
  printList(reverse);
}

main();
