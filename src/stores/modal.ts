export const useModalState = createGlobalState(
    () => {
        const isShowModal = ref(false);
        const message = ref('');

        const showModal = (newMessage: string) => {
            message.value = newMessage;
            isShowModal.value = true;
        };

        const toggleModal = () => {
            isShowModal.value = !isShowModal.value;
        };

        return {
            isShowModal,
            message,
            toggleModal,
            showModal,
        };
    },
);
