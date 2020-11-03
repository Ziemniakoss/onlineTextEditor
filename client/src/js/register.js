async function register() {
	const login = document.getElementById("username-input").value
	const password = document.getElementById("password-input").value
	const dto = {
		username: login,
		password: password
	};
	const request = new Request("http://localhost:5000/register", {
		method: "POST",
		body: JSON.stringify(dto),
		headers: new Headers({
			'content-type': 'application/json'
		})
	});
	fetch(request).then(result => {
		if (result.status === 400) {
			console.log("Error");
			alert(JSON.stringify(result.body));
		} else if (result.status === 200) {
			window.location.href = "login.html";
		}
	})
}