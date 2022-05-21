import { registerTypeDesc } from "../components/ValueTypes";

export const USIZE_TYPE_DESC = {
    name: 'usize',
    defaultValue: 0,
    // Default String renderer.
};

export const VEC_TYPE_DESC = {
    name: 'Vec',
    renderer: 'Vec',
};

registerTypeDesc(USIZE_TYPE_DESC);
registerTypeDesc(VEC_TYPE_DESC);
