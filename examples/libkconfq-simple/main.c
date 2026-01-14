#include <stdio.h>
#include <stdlib.h>

#include <kconfq/kconfq.h>

int main(void) {
  char *kconf_path;

  enum KconfqResult res = kconfq_locate_config(&kconf_path);

  if (res != KCONFQ_RESULT_SUCCESS) {
    fprintf(stderr, "failed to locate kernel config: %s\n",
            kconfq_result_strerror(res));
    goto error;
  }

  printf("%s\n", kconf_path);

error:
  kconfq_free_string(kconf_path);
  return EXIT_SUCCESS;
}
.
