document.addEventListener('DOMContentLoaded', function () {
    const langButtons = {
        en: document.getElementById('en-button'),
        fr: document.getElementById('fr-button')
    };

    const setLanguage = (lang) => {
        document.querySelectorAll('[data-en]').forEach(el => {
            el.innerHTML = el.getAttribute(`data-${lang}`);
        });
    };

    langButtons.en.addEventListener('click', () => setLanguage('en'));
    langButtons.fr.addEventListener('click', () => setLanguage('fr'));
});