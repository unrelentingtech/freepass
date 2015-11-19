#include <stdint.h>
#include "../rusterpassword/capi/rusterpassword_capi.h"

typedef struct vault_S vault_t;
typedef struct string_iter_S string_iter_t;

void freepass_init();

secstr_t* freepass_gen_outer_key(const secstr_t*);
secstr_t* freepass_gen_entries_key(const secstr_t*);
void freepass_free_outer_key(const secstr_t*);
void freepass_free_entires_key(const secstr_t*);

vault_t* freepass_open_vault(const char*, const secstr_t*);
vault_t* freepass_new_vault();
void freepass_close_vault(const vault_t*);

string_iter_t* freepass_vault_get_entry_names_iterator(const vault_t*);
char* freepass_entry_names_iterator_next(string_iter_t*);
void freepass_free_entry_name(char*);
void freepass_free_entry_names_iterator(string_iter_t*);
