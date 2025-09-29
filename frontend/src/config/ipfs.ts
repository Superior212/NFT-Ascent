import {PinataSDK} from "pinata";

export const pinata = new PinataSDK({
    pinataGateway: import.meta.env.VITE_PINATA_GATEWAY,
    pinataJwt: import.meta.env.VITE_PINATA_JWT,
})