import unittest
from subprocess import Popen, PIPE

from snapshot import *


class TestE2E(unittest.TestCase):
    maxDiff = None

    def setUp(self):
        self.db = Popen('target/debug/db-tutorial-rust.exe', stdin=PIPE, stdout=PIPE)

    def tearDown(self):
        self.db.terminate()
        self.db.wait()

    def run_commands(self, commands):
        return self.db.communicate('\n'.join(commands).encode())

    def test__inserts_and_retrieves_a_row(self):
        out, _ = self.run_commands([
            'insert 0 user0 person0@example.com',
            'select',
            '.exit',
        ])

        self.assertEqual(out.decode(), snapshot__inserts_and_retrieves_a_row)

    def test__prints_error_message_when_table_is_full(self):
        out, _ = self.run_commands([
            *(f'insert {i} user{i} person{i}@example.com' for i in range(1401)),
            '.exit',
        ])

        self.assertEqual(out.decode(), snapshot__prints_error_message_when_table_is_full)

    def test__allows_inserting_strings_that_are_the_maximum_length(self):
        username = 'X' * 32
        email = 'X' * 255

        out, _ = self.run_commands([
            f'insert 0 {username} {email}',
            'select',
            '.exit',
        ])

        self.assertEqual(out.decode(), snapshot__allows_inserting_strings_that_are_the_maximum_length)

    def test__prints_error_message_if_username_is_too_long(self):
        username = 'X' * 33
        email = 'X' * 255

        out, _ = self.run_commands([
            f'insert 0 {username} {email}',
            '.exit',
        ])

        self.assertEqual(out.decode(), snapshot__prints_error_message_if_username_is_too_long)

    def test__prints_error_message_if_email_is_too_long(self):
        username = 'X' * 32
        email = 'X' * 256

        out, _ = self.run_commands([
            f'insert 0 {username} {email}',
            '.exit',
        ])

        self.assertEqual(out.decode(), snapshot__prints_error_message_if_email_is_too_long)

    def test__prints_error_message_if_id_is_negative(self):
        out, _ = self.run_commands([
            'insert -1 user-1 person-1@example.com',
            '.exit',
        ])

        self.assertEqual(out.decode(), snapshot__prints_error_message_if_id_is_negative)


if __name__ == '__main__':
    unittest.main()
