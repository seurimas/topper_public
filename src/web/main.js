import './main.scss';

import React, { useState } from 'react';
import ReactDOM from 'react-dom';

import { invoke_target } from './api.js';

const MainView = () => {
    const [target, setTarget] = useState('');
    return (<div onClick={invoke_target(setTarget)}>Current target: {target}</div>);
};

ReactDOM.render(<MainView/>, document.getElementById('app'));
