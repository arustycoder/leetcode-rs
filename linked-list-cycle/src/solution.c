#include<stdbool.h>
#include<stdlib.h>
#include<stdio.h>

struct LinkNode {
    int val;
    struct LinkNode *next;
};

int build_from_array(struct LinkNode **head, int array[], unsigned long len) {
   if (!head || !len || !array)
       return 0;

   struct LinkNode *tmp = malloc(sizeof(struct LinkNode));
   if (!tmp) {return -1;}
   tmp->val = array[0];
   tmp->next = NULL;

   *head = tmp;

   for (unsigned long i = 1; i<len; i++) {
       struct LinkNode *node = malloc(sizeof (struct LinkNode));
       if (!node) {return -1;}
       node->val = array[i];
       node->next = NULL;

       tmp->next = node;
       tmp = node;
   }

   return 0;
}

void show(struct LinkNode *head) {
    while (head) {
        printf("%d ", head->val);
        head = head->next;
    }
    printf("\n");
}

void append(struct LinkNode *head, struct LinkNode *node) {
    while (head->next) {
        head = head->next;
    }
    head->next = node;
}

struct LinkNode *find(struct LinkNode *head, int val) {
    while (head) {
        if (head->val == val) {
            return head;
        }
        head = head->next;
    }
    return NULL;
}

bool hasCycle(struct LinkNode *head) {
    if (!head) return false;

    struct LinkNode *slow, *fast;
    slow = head;
    fast = head->next;

    while (slow != fast) {
        if (fast == NULL || fast->next->next == NULL)
            return false;

        slow = slow->next;
        fast = fast->next->next;
    }
    return true;
}

bool linked_list_cycle() {
    int array[] = {1, 2, 3, 4, 5};
    struct LinkNode *head = NULL;
    build_from_array(&head, array, sizeof(array)/sizeof(array[0]));
    //show(head);
    struct LinkNode *node3 = find(head, 3);
    append(head, node3);
    return hasCycle(head);
}