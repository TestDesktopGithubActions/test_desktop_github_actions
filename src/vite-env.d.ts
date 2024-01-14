/// <reference types="vite/client" />
declare global {
    export type Writable<T> = {
        -readonly [P in keyof T]: T[P];
    };

    type Nullable<T> = T | null;
    type Recordable<T = any> = Record<string, T>;
    type ReadonlyRecordable<T = any> = {
        readonly [key: string]: T;
    };
    type Indexable<T = any> = {
        [key: string]: T;
    };
    type DeepPartial<T> = {
        [P in keyof T]?: DeepPartial<T[P]>;
    };
    type TimeoutHandle = ReturnType<typeof setTimeout>;
    type IntervalHandle = ReturnType<typeof setInterval>;

    interface ChangeEvent extends Event {
        target: HTMLInputElement;
    }

    interface WheelEvent {
        path?: EventTarget[];
    }

    function parseInt(s: string | number, radix?: number): number;

    function parseFloat(s: string | number): number;
}

export {};
