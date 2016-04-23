#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include "../rusterpassword/capi/rusterpassword_capi.h"

typedef struct vault_S vault_t;
typedef struct string_iter_S string_iter_t;
typedef struct {
	uint8_t *data;
	size_t len;
	size_t cap;
} vector_t;

void freepass_init();

secstr_t* freepass_gen_outer_key(const secstr_t*);
secstr_t* freepass_gen_entries_key(const secstr_t*);
void freepass_free_outer_key(const secstr_t*);
void freepass_free_entries_key(const secstr_t*);

vault_t* freepass_open_vault(const char*, const secstr_t*, const secstr_t*);
vault_t* freepass_new_vault(const secstr_t*, const secstr_t*);
void freepass_close_vault(const vault_t*);

string_iter_t* freepass_vault_get_entry_names_iterator(const vault_t*);
char* freepass_entry_names_iterator_next(string_iter_t*);
void freepass_free_entry_name(char*);
void freepass_free_entry_names_iterator(string_iter_t*);

vector_t freepass_vault_get_entry_cbor(const vault_t*, const char*);
void freepass_free_entry_cbor(vector_t);
vector_t freepass_vault_put_entry_cbor(const vault_t*, const char*, uint8_t*, size_t);

#ifdef __cplusplus
}
#endif
