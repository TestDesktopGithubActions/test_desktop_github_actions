import { type App } from 'vue';
import { type ValidationArgs } from '@vuelidate/core';

export type UserModule = (app: App) => void;

export interface SharpFormField {
    name: string;
    label?: string;
    icon?: string;
    type: string;
    value: string;
    mask?: boolean;
    placeholder?: string;
    disabled?: boolean | Ref<any>;
    rules?: ValidationArgs<any>;
}

export interface RouteNode {
    countryCode: string;
    nodes: NodeData[];
    delay: number;
    minDelayNodeIndex: number;
    defaultCountryCode?: string;
}

export interface NodeData {
    city: string;
    guid: string;
    ip: string;
    node_port: number;
}

export interface ModalElement extends HTMLDialogElement {
    showModal: () => void;
}
