#include <stdio.h>
#include <stdlib.h>

#include <kconfq/kconfq.h>

int main(void) {
  KconfqResult res;
  int exit_status = EXIT_SUCCESS;

  const KconfqConfig *config = NULL;
  res = kconfq_locate_config(&config);
  if (res != KCONFQ_RESULT_SUCCESS) {
    fprintf(stderr, "Error: %s\n", kconfq_result_strerror(res));
    exit_status = EXIT_FAILURE;
    goto cleanup1;
  }

  const char *config_path = NULL;
  res = kconfq_config_path(config, &config_path);
  if (res != KCONFQ_RESULT_SUCCESS) {
    fprintf(stderr, "Error: %s\n", kconfq_result_strerror(res));
    exit_status = EXIT_FAILURE;
    goto cleanup2;
  }

  printf("Path to config: %s\n", config_path);

  const char *line = NULL;
  res = kconfq_find_line(config, "CONFIG_CC_VERSION_TEXT", &line);
  if (res != KCONFQ_RESULT_SUCCESS) {
    fprintf(stderr, "Error: %s\n", kconfq_result_strerror(res));
    exit_status = EXIT_FAILURE;
    goto cleanup3;
  }

  printf("Line: %s\n", line);

  const char *entry_value = NULL;
  res = kconfq_find_value(config, "CONFIG_CC_VERSION_TEXT", &entry_value);
  if (res != KCONFQ_RESULT_SUCCESS) {
    fprintf(stderr, "Error: %s\n", kconfq_result_strerror(res));
    exit_status = EXIT_FAILURE;
    goto cleanup4;
  }

  printf("Entry value: %s\n", entry_value);

cleanup4:
  kconfq_free_string(entry_value);
cleanup3:
  kconfq_free_string(line);
cleanup2:
  kconfq_free_string(config_path);
cleanup1:
  kconfq_free_config(config);

  return exit_status;
}
