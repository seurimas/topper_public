import { registerTypeDesc } from "../components/ValueTypes";
import { getOptionOf } from "./Option";

export const USIZE_TYPE_DESC = {
    name: 'usize',
    defaultValue: 0,
    // Default String renderer.
};

export const BOOL_TYPE_DESC = {
    name: 'bool',
    defaultValue: false,
    renderer: 'Boolean',
};

export const STRING_TYPE_DESC = {
    name: 'String',
    defaultValue: '',
    renderer: 'String',
};

export const VEC_TYPE_DESC = {
    name: 'Vec',
    renderer: 'Vec',
};

registerTypeDesc(USIZE_TYPE_DESC);
registerTypeDesc(BOOL_TYPE_DESC);
registerTypeDesc(VEC_TYPE_DESC);
registerTypeDesc(getOptionOf(STRING_TYPE_DESC));