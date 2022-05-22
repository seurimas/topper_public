export const getOptionName = (name) => `Option<${name}>`;

export const getOptionOf = (typeDesc) => ({
    name: getOptionName(typeDesc.name),
    variants: [{
        name: 'None',
    }, {
        name: 'Some',
        fields: [typeDesc.name],
    }],
});