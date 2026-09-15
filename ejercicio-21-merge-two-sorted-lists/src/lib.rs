#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
      ListNode {
        next: None,
        val
      }
    }
}

fn new_list(list: Vec<i32>) -> Option<Box<ListNode>> {
    let mut cant_elementos = list.len();
    let mut lista_nodos: Vec<Box<ListNode>> = Vec::with_capacity(cant_elementos);
    for val in list {
        lista_nodos.push(Box::new(ListNode::new(val)));
    }

    while let (Some(ultimo), Some(ante_ultimo)) = (lista_nodos.pop_if(|_| cant_elementos > 1), lista_nodos.last_mut()) {
        (*ante_ultimo).next = Some(ultimo);
        cant_elementos -= 1;
    }

    if let Some(ultimo) = lista_nodos.get(0) {
        Some(ultimo.clone())
    } else {
        None
    }
}

pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut nodo_siguiente1 = &list1;
    let mut nodo_siguiente2 = &list2;
    let mut resultado: Vec<i32> = Vec::new();

    while let (Some(nodo1), Some(nodo2)) = (nodo_siguiente1, nodo_siguiente2) {
        if nodo1.val == nodo2.val {
            nodo_siguiente1 = &nodo1.next;
            resultado.push(nodo1.val);

            nodo_siguiente2 = &nodo2.next;
            resultado.push(nodo2.val);

        } else if nodo1.val < nodo2.val {
            nodo_siguiente1 = &nodo1.next;
            resultado.push(nodo1.val);

        } else {
            nodo_siguiente2 = &nodo2.next;
            resultado.push(nodo2.val);
        }
    }

    while let Some(nodo) = nodo_siguiente1 {
        nodo_siguiente1 = &nodo.next;
        resultado.push(nodo.val);
    }

    while let Some(nodo) = nodo_siguiente2 {
        nodo_siguiente2 = &nodo.next;
        resultado.push(nodo.val);
    }

    new_list(resultado)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_nodo(val: i32, next: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut nodo = ListNode::new(val);
        nodo.next = next;
        Some(Box::new(nodo))
    }

    #[test]
    fn ejemplo_1() {
        let resultado = merge_two_lists(
                new_nodo(1, new_nodo(2, new_nodo(4, None))),
                new_nodo(1, new_nodo(3, new_nodo(4, None))),
        );

        assert_eq!(resultado, new_nodo(1, new_nodo(1, new_nodo(2, new_nodo(3, new_nodo(4, new_nodo(4, None)))))));
    }

    #[test]
    fn ejemplo_2() {
        let resultado = merge_two_lists(None, None);
        assert_eq!(resultado, None);
    }

    #[test]
    fn ejemplo_3() {
        let resultado = merge_two_lists(
            None,
            new_nodo(0, None),
        );

        assert_eq!(resultado, new_nodo(0, None));
    }
}
