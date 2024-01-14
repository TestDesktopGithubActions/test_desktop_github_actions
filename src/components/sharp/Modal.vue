<script lang="ts" setup>
    const modalElementRef = ref<ModalElement | null>(null);
    const { isShowModal, message, toggleModal } = useModalState();

    watch(isShowModal, (value) => {
        if (value) {
            modalElementRef.value?.showModal();
        }
    });
</script>

<template>
    <dialog ref="modalElementRef" class="modal rd-24px">
        <div class="modal-box">
            <h3 class="font-bold text-lg">
                {{ $t('modal.notification') }}
            </h3>
            <p class="py-4">
                {{ message }}
            </p>
            <div class="modal-action">
                <form method="dialog">
                    <!-- if there is a button in form, it will close the modal -->
                    <button class="btn" @click="toggleModal">
                        {{ $t('modal.close') }}
                    </button>
                </form>
            </div>
        </div>
    </dialog>
</template>

<style lang="scss">
dialog::backdrop {
    border-radius: 24px;
}
</style>
