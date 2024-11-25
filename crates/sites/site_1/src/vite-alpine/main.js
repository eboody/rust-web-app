import './style.css';
import Alpine from 'alpinejs';

window.Alpine = Alpine;

Alpine.data('app', () => ({
  count: 0,
}))

	function setCookie(name, value, days) {
		const date = new Date();
		date.setTime(date.getTime() + days * 24 * 60 * 60 * 1000);
		document.cookie = `${name}=${value}; expires=${date.toUTCString()}; path=/;`;
	}


	function getCookie(name) {
		const value = `; ${document.cookie}`;
		const parts = value.split(`; ${name}=`);
		if (parts.length === 2) {
			return parts.pop().split(";").shift() === "true";
		}
		return false;
	}

window.setCookie = setCookie;
window.getCookie = getCookie;
Alpine.start();
