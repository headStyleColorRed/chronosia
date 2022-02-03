import axios from "axios"

const SERVER_PROTOCOL = process.env.VUE_APP_PROTOCOL
const SERVER_IP = process.env.VUE_APP_SERVER_IP
const SERVER_API = process.env.VUE_APP_LOGIN_API


async function logUser(username, password) {
    let user ={
        "email": username,
        "password": password
	}
	
	let promise = new Promise((resolve, reject) => {
		axios.post(`${SERVER_PROTOCOL}://${SERVER_IP}${SERVER_API}/login/log_user`, user)
        .then((res) => {  
			resolve(res.data)})
		.catch((res) => {
			reject(res.data)})
	})
	
	let result = await promise
	return result
}

async function registerUser(username, password, passwordConfirmation) {
    let user = {
        "email": username,
		"password": password,
		"passwordConfirmation": passwordConfirmation
	}
	
	let promise = new Promise((resolve, reject) => {
		axios.post(`${SERVER_PROTOCOL}://${SERVER_IP}${SERVER_API}/register/register_user`, user)
        .then((res) => {  
			resolve(res.data)})
		.catch((res) => {
			reject(res.data)})
	})
	
	let result = await promise
	return result
}

async function logOut(username, password) {
    let user ={
        "username": username,
        "password": password
	}
	
	let promise = new Promise((resolve, reject) => {
		axios.post(`${SERVER_PROTOCOL}://${SERVER_IP}${SERVER_API}/logout/logout_user`, user)
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