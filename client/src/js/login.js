async function login() {
	const login = document.getElementById("username-input").value
	const password = document.getElementById("password-input").value
	const dto = {
		username: login,
		password: password
	};
	const request = new Request("http://localhost:5000/login", {
		method: "POST",
		body: JSON.stringify(dto),
		headers: new Headers({
			'content-type': 'application/json'
		}),
		credentials: "include",

	});
	fetch(request).then(result => {
		if (result.status === 400) {
			console.log("Error");
		} else if (result.status === 200) {
			window.location.href = "projects-list.html";
		}
	})


}

async function logout() {
	const request1 = new Request("http://localhost:5000/logout", {
		method: "POST",
		credentials: 'include',
		headers: new Headers({
			'content-type': 'application/json'
		})
	});
	console.log("Now logout")
	fetch(request1).then(result => {
		if (result.status !== 200) {
			console.log("Error");
		} else {
			console.log("ok")
		}
	})
}