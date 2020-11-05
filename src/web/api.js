window.promises = {};

const invoke = (str, promise_id = null) => {
    if (promise_id !== null) {
	const promise = new Promise((resolve) => {
	    window.promises[promise_id] = {
		resolve,
	    };
	    external.invoke(str);
	});
	window.promises[promise_id].promise = promise;
	return promise;
    } else {
	external.invoke(str);
    }
};

export const invoke_target = (setTarget) => () => {
    invoke("test_one", "test_one").then(setTarget);
};

window.resolve_promise = (promise_id, result) => {
    if (window.promises[promise_id] !== undefined && !window.promises[promise_id].resolved) {
	window.promises[promise_id].resolve(result);
	window.promises[promise_id].resolved = true;
    }
};
