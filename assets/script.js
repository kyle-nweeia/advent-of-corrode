function setAction(form) {
    form.action = `/submit/${form.year.value}/${form.day.value}/${form.part.value}`;
}
