import axios from "axios"
import store from "../../store/index"

const Environments = {
	docker: "login-server",
	local: "localhost",
	currentEnvironment: window.location.hostname
}

const currentEnvironment = Environments.currentEnvironment

async function logUser(email, password) {
    let user = {
        "email": email,
        "password": password
	}

	console.log("Environment: " + currentEnvironment);
	let promise = new Promise((resolve, reject) => {
		axios.post(`http://${currentEnvironment}:8888/login/log_user`, user)
        .then((res) => {  
			resolve(res.data)})
		.catch((res) => {
			reject(res.data)})
	})
	
	let result = await promise
	return result
}

async function registerUser(email, username, password, passwordConfirmation) {
    let user ={
		"email": email,
        "username": username,
		"password": password,
		"passwordConfirmation": passwordConfirmation
	}
	
	let promise = new Promise((resolve, reject) => {
		axios.post(`http://${currentEnvironment}:8888/register/register_user`, user)
        .then((res) => {  
			resolve(res.data)})
		.catch((res) => {
			reject(res.data)})
	})
	
	let result = await promise
	return result
}

async function logOut(user) {
    let userData = {
        "email": user.email,
        "password": store.getters.password
	}
	
	let promise = new Promise((resolve, reject) => {
		axios.post(`http://${currentEnvironment}:8888/logout/logout_user`, userData)
        .then((res) => {  
			resolve(res.data)})
		.catch((res) => {
			reject(res.data)})
	})
	
	let result = await promise
	return result
}

export default {
    logUser,
	registerUser,
	logOut,
}