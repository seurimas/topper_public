export const ADD_TO_VEC = 'ADD_TO_VEC';
export const REMOVE_FROM_VEC = 'REMOVE_FROM_VEC';

export const SET_ENUM_VARIANT = 'SET_ENUM_VARIANT';
export const SET_VALUE = 'SET_VALUE';

export const addToVec = (path) => ({
    type: ADD_TO_VEC,
    path,
});

export const removeFromVec = (path, index) => ({
    type: REMOVE_FROM_VEC,
    path,
    index,
});

export const setEnumVariant = (path, variant) => ({
    type: SET_ENUM_VARIANT,
    path,
    variant,
});

export const setValue = (path, value) => ({
    type: SET_VALUE,
    path,
    value,
});