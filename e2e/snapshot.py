snapshot__inserts_and_retrieves_a_row = '''
db > db > Row { id: 0, username: "user0", email: "person0@example.com" }
db > 
'''[1:-1]

snapshot__prints_error_message_when_table_is_full = f'''
{'db > ' * 1401}Error: Table full.
db > 
'''[1:-1]

snapshot__allows_inserting_strings_that_are_the_maximum_length = '''
db > db > Row { id: 0, username: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX", email: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX" }
db > 
'''[1:-1]

snapshot__prints_error_message_if_username_is_too_long = '''
db > Error: 'username' is too long.
db > 
'''[1:-1]

snapshot__prints_error_message_if_email_is_too_long = '''
db > Error: 'email' is too long.
db > 
'''[1:-1]

snapshot__prints_error_message_if_id_is_negative = '''
db > Syntax Error: Could not parse statement.
db > 
'''[1:-1]
