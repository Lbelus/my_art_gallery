document.addEventListener('DOMContentLoaded', function () {
    const cookieWarning = document.getElementById('cookie-warning');
    const acceptCookiesButton = document.getElementById('accept-cookies');

    if (!document.cookie.includes('cookie_accepted=true')) {
        cookieWarning.style.display = 'block';
    }

    acceptCookiesButton.addEventListener('click', function () {
        document.cookie = 'cookie_accepted=true; path=/; max-age=31536000';
        cookieWarning.style.display = 'none';
    });
});