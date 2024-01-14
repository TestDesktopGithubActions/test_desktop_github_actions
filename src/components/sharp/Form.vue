<script lang="ts" setup>
    import { type ValidationArgs, useVuelidate } from '@vuelidate/core';
    import { helpers, sameAs } from '@vuelidate/validators';
    import type { Directive } from 'vue';
    import { type SharpFormField } from '~/types';

    interface SharpFormProps {
        fields: SharpFormField[];
        submitText: string;
        loading?: boolean;
    }

    interface InputChangeEvent {
        name: string;
        value: string;
    }

    interface SetErrorEvent {
        name: string;
        message: string;
    }

    interface SharpFormEmits {
        (event: 'inputChange', payload: InputChangeEvent): void;
        (event: 'setError', payload: SetErrorEvent): void;
        (event: 'deleteError', payload: string): void;
        (event: 'submit', payload: Indexable): void;
    }
    const props = defineProps<SharpFormProps>();
    const emit = defineEmits<SharpFormEmits>();

    const state = reactive<Indexable>({});
    const rules = computed<ValidationArgs>(() => {
        const rules: ValidationArgs<any> = {};
        props.fields.forEach((field) => {
            if (field.rules) {
                if (field.rules.sameAsField) {
                    const { $message, $params } = field.rules.sameAsField as any;
                    rules[field.name] = {
                        ...field.rules,
                        sameAsField: helpers.withMessage(
                            $message,
                            sameAs(state[$params.equalTo]),
                        ),
                    };
                } else {
                    rules[field.name] = field.rules;
                }
            } else {
                rules[field.name] = {
                    optional: (value: string) => helpers.req(value) || value === '',
                };
            }
        });
        return rules;
    });

    const v$ = useVuelidate(rules, state);

    const vToggleMask: Directive = {
        mounted(el, binding) {
            const input = document.querySelector(`input[name="${binding.value}"]`);
            if (input) {
                el.addEventListener('click', () => {
                    const inputEl = input as HTMLInputElement;
                    inputEl.type = inputEl.type === 'password' ? 'text' : 'password';

                    const divIcon = el.querySelector('div');
                    if (inputEl.type === 'password') {
                        divIcon.classList.remove('i-carbon:view-off');
                        divIcon.classList.add('i-carbon:view');
                    } else {
                        divIcon.classList.remove('i-carbon:view');
                        divIcon.classList.add('i-carbon:view-off');
                    }
                });
            }
        },
    };

    function handleChange(e: Event, name: string) {
        const input = e.currentTarget as HTMLInputElement;
        const value = input.value;
        emit('inputChange', { name, value });
        validateField(name);
    }

    async function handleSubmit(e: Event) {
        e.preventDefault();
        const isFormCorrect = await v$.value.$validate();
        if (isFormCorrect) {
            emit('submit', state);
            v$.value.$reset();
        }
    }

    function validateField(name: string) {
        const field = v$.value[name];
        if (field) {
            field.$touch();
            if (field.$error) {
                emit('setError', { name, message: field.$errors[0].$message });
            } else {
                emit('deleteError', name);
            }
        }
    }

    watch(
        () => props.fields,
        (fields) => {
            fields.forEach((field) => {
                state[field.name] = field.value;
            });
        },
        { deep: true },
    );

    onMounted(() => {
        props.fields.forEach((field) => {
            state[field.name] = field.value;
        });
    });
</script>

<template>
    <form flex-col gap-16px novalidate @submit.prevent="handleSubmit">
        <div
            v-for="field in fields"
            :key="field.name"
            flex-col gap-5px
        >
            <div
                flex items-center gap-10px py-10px px-14px w-full h-44px rd-8px bg-secondary border="1px solid transparent"
                :class="{ '!border-error': v$[field.name]?.$dirty && v$[field.name]?.$invalid }"
            >
                <input
                    v-model="v$[field.name].$model"
                    class="input-ghost"
                    text="#5D6574 14px"
                    bg-transparent
                    h-full w-full
                    focus:outline-none
                    :type="field.type"
                    :name="field.name"
                    :placeholder="field.placeholder"
                    :class="{ 'text-error': v$[field.name]?.$dirty && v$[field.name]?.$invalid }"
                    :disabled="loading"
                    @input="handleChange($event, field.name)"
                    @blur="v$[field.name]?.$touch"
                >
                <button
                    v-if="field.mask"
                    v-toggle-mask="field.name"
                    type="button"
                >
                    <div class="i-carbon:view c-#5D6574" w-20px h-20px />
                </button>
            </div>
            <small v-for="error of v$[field.name]?.$errors" :key="error.$uid" class="text-error text-12px">
                {{ error.$message }}
            </small>
        </div>
        <button
            type="submit"
            text="16px #FFF"
            font-600 lh-24px
            w-full h-48px rd-10px mt-20px
            important-bg-primary
        >
            <span v-if="loading" class="loading loading-dots loading-md" />
            <template v-else>
                {{ submitText }}
            </template>
        </button>
    </form>
</template>
