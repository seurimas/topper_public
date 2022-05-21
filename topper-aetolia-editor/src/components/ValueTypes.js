import React from 'react';
import { TextField } from "@mui/material";
import { useDispatch, useSelector } from 'react-redux';
import { getEnumField } from '../behavior_tree/reducer_selectors';
import { setValue } from '../behavior_tree/actions';

const StringField = ({ path }) => {
    const dispatch = useDispatch();
    const value = useSelector(getEnumField(path));
    const onChange = (event) => console.log(event) || dispatch(setValue(path, event.target.value));
    return <TextField value={value || ''} onChange={onChange} />;
};

const TYPE_RENDERERS = {
    "String": StringField,
};

export const registerTypeRenderer = (valueType, renderer) => {
    TYPE_RENDERERS[valueType] = renderer;
};

export const getTypeRenderer = (typeDesc) => {
    if (typeDesc && TYPE_RENDERERS[typeDesc.renderer]) {
        return TYPE_RENDERERS[typeDesc.renderer];
    } else if (typeDesc && TYPE_DESCS[typeDesc.name] && TYPE_DESCS[typeDesc.name].renderer) {
        return TYPE_RENDERERS[TYPE_DESCS[typeDesc.name].renderer];
    }
    return StringField;
}

export const TYPE_DESCS = {};

export const registerTypeDesc = (typeDesc) => {
    TYPE_DESCS[typeDesc.name] = typeDesc;
};

export const renderValueOfType = (path, typeDesc) => {
    const fullTypeDesc = (typeof typeDesc === 'string')
        ? TYPE_DESCS[typeDesc]
        : typeDesc;
    const Field = getTypeRenderer(fullTypeDesc);
    return (
        <Field
            path={path}
            typeDesc={fullTypeDesc}
        />
    );
};