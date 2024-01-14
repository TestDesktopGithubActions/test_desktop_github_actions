export async function generateSignature(body: string, nonce: Uint8Array, key: Uint8Array): Promise<string> {
    // Generate timestamp and nonce
    const timestamp = getFormattedTimestamp();

    const hexNonce: string = bytesToHex(nonce);

    // Create the string to be signed
    const unsignedString = `${timestamp}${hexNonce}${await sha256(body)}`;

    // Sign the string using HMAC-SHA256 with the private key
    const base64Signature = await signWithHmacSHA256(unsignedString, key);

    return `RF-BODY-SIG Timestamp="${timestamp}", Nonce="${hexNonce}", Signature="${base64Signature}"`;
}

export async function encryptData(object: any, nonce: Uint8Array, key: Uint8Array): Promise<any> {
    const keyBuffer = await window.crypto.subtle.importKey(
        'raw',
        key,
        { name: 'AES-GCM' },
        false,
        ['encrypt'],
    );

    const encoder = new TextEncoder();
    const data = encoder.encode(JSON.stringify(object));

    const encryptedData = await window.crypto.subtle.encrypt(
        { name: 'AES-GCM', iv: nonce },
        keyBuffer,
        data,
    );

    const result = {
        body: Array.from(new Uint8Array(encryptedData)).map(byte => byte.toString(16).padStart(2, '0')).join(''),
    };

    return result;
}

export async function decryptData(result: string, nonce: Uint8Array, key: Uint8Array): Promise<any> {
    const secretKeyBuffer = await window.crypto.subtle.importKey(
        'raw',
        key,
        { name: 'AES-GCM' },
        false,
        ['decrypt'],
    );

    const encryptedData = hexToUint8Array(result); // 将HEX编码的密文转换为Uint8Array
    const additionalData = new Uint8Array(0); // 附加数据为空，如果有附加数据，请替换为实际附加数据的Uint8Array

    const decryptedData = await window.crypto.subtle.decrypt(
        {
            name: 'AES-GCM',
            iv: nonce,
            additionalData,
            tagLength: 128, // GCM标签长度为128位
        },
        secretKeyBuffer,
        encryptedData,
    );

    const clearText = new TextDecoder().decode(decryptedData);
    return JSON.parse(clearText);
}

// Utility function to generate a random nonce
export function randomNonce(): Uint8Array {
    const nonce = new Uint8Array(12);
    for (let i = 0; i < 12; i++) {
        nonce[i] = Math.floor(Math.random() * 256);
    }
    return nonce;
}

export function parseEncryptKey(token: string): Uint8Array {
    const payloadBase64 = token.split('.')[1];
    const normalizedPayloadBase64 = payloadBase64.replace(/-/g, '+').replace(/_/g, '/');
    const decodedPayload = atob(normalizedPayloadBase64);
    const payload = JSON.parse(decodedPayload);
    const keyArray = payload.key;
    const secretKey = new Uint8Array(keyArray);
    return secretKey;
}

// Utility function to convert Uint8Array to hex string
function bytesToHex(bytes: Uint8Array): string {
    return Array.prototype.map.call(bytes, (byte: number) => {
        return (`0${(byte & 0xFF).toString(16)}`).slice(-2);
    }).join('');
}

// Utility function to convert hex string to Uint8Array
function hexToUint8Array(hex: string): Uint8Array {
    const result = new Uint8Array(hex.length / 2);
    for (let i = 0, j = 0; i < hex.length; i += 2, j++) {
        const byte = Number.parseInt(hex.slice(i, i + 2), 16);
        if (!Number.isNaN(byte)) {
            result[j] = byte;
        } else {
            throw new TypeError(`Invalid hex string: ${hex}`);
        }
    }
    return result;
}

// Utility function to calculate SHA-256 hash
async function sha256(message: string) {
    const encoder = new TextEncoder();
    const data = encoder.encode(message);
    const hashBuffer = await crypto.subtle.digest('SHA-256', data);
    const hashArray = Array.from(new Uint8Array(hashBuffer));
    return hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
}

// Utility function to sign a string using HMAC-SHA256
async function signWithHmacSHA256(message: string, key: Uint8Array) {
    const encoder = new TextEncoder();
    const messageData = encoder.encode(message);

    // Import the key
    const importedKey = await crypto.subtle.importKey(
        'raw',
        key,
        { name: 'HMAC', hash: { name: 'SHA-256' } },
        false,
        ['sign'],
    );

    // Sign the message
    const signature = await crypto.subtle.sign('HMAC', importedKey, messageData);

    // Convert the signature to base64
    const signatureArray = Array.from(new Uint8Array(signature));
    return btoa(signatureArray.map(b => String.fromCharCode(b)).join(''));
}
