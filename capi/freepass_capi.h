#include <stdint.h>
#include "../rusterpassword/capi/rusterpassword_capi.h"

typedef struct vault_S vault_t;

void freepass_init();

secstr_t* freepass_gen_outer_key(const secstr_t*);
secstr_t* freepass_gen_entries_key(const secstr_t*);
void freepass_free_outer_key(const secstr_t*);
void freepass_free_entires_key(const secstr_t*);

vault_t* freepass_open_vault(const char*, const secstr_t*);
void freepass_close_vault(const vault_t*);
