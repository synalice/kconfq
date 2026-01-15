#include <stdio.h>
#include <stdlib.h>

#include <kconfq/kconfq.h>

int main(void) {
  KconfqError *err;
  char *kconf_path;
  int exit_status = EXIT_SUCCESS;

  kconfq_locate_config(&kconf_path, &err);

  if (err != NULL) {
    fprintf(stderr, "Error:\n");

    int err_num = 1;

    for (KconfqError *e = err; e != NULL; e = e->cause) {
      fprintf(stderr, "%3d. %s\n", err_num, kconfq_error_message(e));
      err_num += 1;
    }

    exit_status = EXIT_FAILURE;
    goto cleanup;
  }

  printf("%s\n", kconf_path);

cleanup:
  kconfq_free_string(kconf_path);
  kconfq_free_error(err);

  return exit_status;
}
