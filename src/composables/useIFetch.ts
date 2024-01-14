import { ResponseType, fetch } from '@tauri-apps/api/http';

interface EchoResponse {
    body: string;
    nonce: number[];
}

const defaultSecretKey = new Uint8Array([55, 250, 217, 28, 25, 199, 172, 105, 94, 84, 168, 152, 235, 111, 190, 229, 165, 100, 125, 55, 131, 55, 81, 131, 161, 50, 120, 23, 99, 102, 33, 51]);

export function useIFetch() {
    async function echo(endpoint: string) {
        const url = `http://${endpoint}/echo`;

        const signature = await generateSignature(
            '/echo',
            randomNonce(),
            defaultSecretKey,
        );

        const requestTimestamp = Date.now();
        const response = await fetch<EchoResponse>(url, {
            method: 'GET',
            headers: {
                'content-type': 'application/json',
                signature,
            },
            responseType: ResponseType.JSON,
        });
        const responseTimestamp = Date.now();
        if (response.ok) {
            const { body, nonce } = response.data;
            const { data } = await decryptData(body, new Uint8Array(nonce), defaultSecretKey);
            return {
                delay: responseTimestamp - requestTimestamp,
                data,
            };
        } else {
            return {
                delay: responseTimestamp - requestTimestamp,
                data: null,
                status: response.status,
            };
        }
    }

    return {
        echo,
    };
}
