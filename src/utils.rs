pub mod scope_channel {
    use yew::html::Scope;
    use std::rc::Rc;
    use once_cell::unsync::OnceCell;

    pub struct Sender<T: yew::Component>(Rc<OnceCell<Scope<T>>>);
    pub struct Receiver<T: yew::Component>(Rc<OnceCell<Scope<T>>>);

    impl<T: yew::Component> Sender<T> {
        pub fn send(&self,val: Scope<T>) {
            (self.0).set(val);
        }
    }

    impl<T: yew::Component> Receiver<T> {
        pub fn recv(&self) -> &Scope<T> {
            (self.0).get().unwrap()
        }
    }

    impl<T: yew::Component> std::cmp::PartialEq for Sender<T> {
        fn eq(&self, _other: &Self) -> bool { true }
    }

    impl<T: yew::Component> std::cmp::PartialEq for Receiver<T> {
        fn eq(&self, _other: &Self) -> bool { true }
    }

    pub fn scope_channel<T: yew::Component>() -> (Sender<T>,Receiver<T>) {
        let inner = Rc::new(OnceCell::new());
        (Sender(inner.clone()),Receiver(inner))
    }
}