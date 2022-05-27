import React from 'react';
import { Checkbox, TextField } from "@mui/material";
import { useDispatch, useSelector } from 'react-redux';
import { getEnumField } from '../behavior_tree/reducer_selectors';
import { setValue } from '../behavior_tree/actions';

const StringField = ({ treeName, path }) => {
    const dispatch = useDispatch();
    const value = useSelector(getEnumField(treeName, path));
    const onChange = (event) => dispatch(setValue(treeName, path, event.target.value));
    return <TextField value={value || ''} onChange={onChange} />;
};

const BooleanField = ({ treeName, path }) => {
    const dispatch = useDispatch();
    const value = useSelector(getEnumField(treeName, path));
    const onChange = (event) => dispatch(setValue(treeName, path, event.target.checked));
    return <Checkbox checked={value || false} onChange={onChange} />;
};

const TYPE_RENDERERS = {
    "String": StringField,
    "Boolean": BooleanField,
};

export const registerTypeRenderer = (valueType, renderer) => {
    TYPE_RENDERERS[valueType] = renderer;
};

export const getTypeRenderer = (typeDesc) => {
    if (typeDesc && TYPE_RENDERERS[typeDesc.renderer]) {
        return TYPE_RENDERERS[typeDesc.renderer];
    } else if (typeDesc && TYPE_DESCS[typeDesc.name] && TYPE_DESCS[typeDesc.name].renderer) {
        return TYPE_RENDERERS[TYPE_DESCS[typeDesc.name].renderer];
    } else if (typeDesc && typeDesc.variants) {
        return TYPE_RENDERERS.Enum;
    }
    return StringField;
}

export const TYPE_DESCS = {};
export const VARIANT_TYPES = {};

export const registerTypeDesc = (typeDesc) => {
    console.log(typeDesc);
    TYPE_DESCS[typeDesc.name] = typeDesc;
    if (typeDesc.variants) {
        typeDesc.variants.forEach(({ name }) => {
            VARIANT_TYPES[name] = typeDesc.name;
        });
    }
};

export const renderValueOfType = (treeName, path, typeDesc) => {
    const fullTypeDesc = (typeof typeDesc === 'string')
        ? TYPE_DESCS[typeDesc]
        : typeDesc;
    const Field = getTypeRenderer(fullTypeDesc);
    return (
        <Field
            key={path.join(',')}
            treeName={treeName}
            path={path}
            typeDesc={fullTypeDesc}
        />
    );
};

export const getTypeFromVariant = (variantName) => {
    return TYPE_DESCS[VARIANT_TYPES[variantName]];
}