#   NQRUSTBACKUP - Backup Archiving REcovery Open Sourced
#
#   Copyright (C) 2016-2024 NQRustBackup GmbH & Co. KG
#
#   This program is Free Software; you can redistribute it and/or
#   modify it under the terms of version three of the GNU Affero General Public
#   License as published by the Free Software Foundation and included
#   in the file LICENSE.
#
#   This program is distributed in the hope that it will be useful, but
#   WITHOUT ANY WARRANTY; without even the implied warranty of
#   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
#   Affero General Public License for more details.
#
#   You should have received a copy of the GNU Affero General Public License
#   along with this program; if not, write to the Free Software
#   Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA
#   02110-1301, USA.

"""
Send and receive the response to NQRustBackup Director Daemon Console interface.
"""

from nqrustbackup.bsock.connectiontype import ConnectionType
from nqrustbackup.bsock.lowlevel import LowLevel
from nqrustbackup.bsock.protocolmessageids import ProtocolMessageIds
from nqrustbackup.bsock.protocolmessages import ProtocolMessages
from nqrustbackup.bsock.protocolversions import ProtocolVersions
from nqrustbackup.bsock.tlsversionparser import TlsVersionParser
import nqrustbackup.exceptions


class DirectorConsole(LowLevel):
    """Send and receive the response to NQRustBackup Director Daemon Console interface.

    Example:
       >>> import nqrustbackup.bsock
       >>> directorconsole=nqrustbackup.bsock.DirectorConsole(address='localhost', port=9101, name='user1', password='secret')
       >>> print(directorconsole.call('help').decode("utf-8"))
    """

    @staticmethod
    def argparser_add_default_command_line_arguments(argparser):
        """Extend argparser with :py:class:`DirectorConsole` specific parameter.

        Every command line program must offer a similar set of parameter
        to connect to a NQRustBackup Director.
        This method adds the required parameter to an existing ArgParser instance.
        Parameter required to initialize a DirectorConsole class
        are stored in variables prefixed with ``NQRUSTBACKUP_``.

        Use :py:func:`nqrustbackup.bsock.lowlevel.LowLevel.argparser_get_nqrustbackup_parameter` to retrieve the relevant parameter
        (with the ``NQRUSTBACKUP_`` prefix removed).

        Example:
           >>> argparser = argparse.ArgumentParser(description="Console to NQRustBackup Director.")
           >>> DirectorConsole.argparser_add_default_command_line_arguments(argparser)
           >>> args = argparser.parse_args()
           >>> nqrustbackup_args = DirectorConsole.argparser_get_nqrustbackup_parameter(args)
           >>> director = DirectorConsole(**nqrustbackup_args)

        Args:
          argparser (ArgParser or ConfigArgParser): (Config)ArgParser instance.
        """

        group = argparser.add_argument_group(title="NQRustBackup Director connection options")

        group.add_argument(
            "--name",
            default="*UserAgent*",
            help='use this to access a specific NQRustBackup director named console. Otherwise it connects to the default console ("%(default)s").',
            dest="NQRUSTBACKUP_name",
        )

        group.add_argument(
            "-p",
            "--password",
            help="Password to authenticate to a NQRustBackup Director console.",
            required=True,
            dest="NQRUSTBACKUP_password",
        )

        group.add_argument(
            "--port",
            default=9101,
            help="NQRustBackup Director network port. Default: %(default)s.",
            dest="NQRUSTBACKUP_port",
        )

        # argparser.add_argument('--dirname', help="NQRustBackup Director name")
        group.add_argument(
            "--address",
            default="localhost",
            help="NQRustBackup Director network address. Default: %(default)s.",
            dest="NQRUSTBACKUP_address",
        )

        group.add_argument(
            "--timeout",
            type=int,
            help="Timeout (in seconds) for the connection to the NQRustBackup Director.",
            dest="NQRUSTBACKUP_timeout",
        )

        group.add_argument(
            "--protocolversion",
            type=int,
            choices=set(protocolversion.value for protocolversion in ProtocolVersions)
            | set([0]),
            default=ProtocolVersions.last.value,
            help="Specify the NQRustBackup console protocol version (0: auto-detect, %(default)s: latest). Default: %(default)s.",
            dest="NQRUSTBACKUP_protocolversion",
        )

        group.add_argument(
            "--pam-username",
            help="Username to authenticate against PAM on top off the normal authentication.",
            dest="NQRUSTBACKUP_pam_username",
        )

        group.add_argument(
            "--pam-password",
            help="Password to authenticate against PAM on top off the normal authentication.",
            dest="NQRUSTBACKUP_pam_password",
        )

        group.add_argument(
            "--tls-psk-require",
            help="Allow only encrypted connections. Default: %(default)s.",
            action="store_true",
            dest="NQRUSTBACKUP_tls_psk_require",
        )

        TlsVersionParser().add_argument(group)

    def __init__(
        self,
        address="localhost",
        port=9101,
        timeout=None,
        dirname=None,
        name="*UserAgent*",
        password=None,
        protocolversion=None,
        pam_username=None,
        pam_password=None,
        tls_psk_enable=True,
        tls_psk_require=False,
        tls_version=None,
    ):
        """\

        Args:
           address (str): Address of the NQRustBackup Director (hostname or IP).

           port (int): Port number of the NQRustBackup Director.

           timeout (int, optional):
              Timeout for the connection to the director. Default: OS dependent

           dirname (str, optional):
              Name of the NQRustBackup Director. Deprecated, normally not required.

           name (str, optional):
              Name of the Director Console. Leave empty when connecting to the NQRustBackup Default Console.

           password  (str, nqrustbackup.util.Password):
              Password, in cleartext or as Password object.

           protocolversion (None, nqrustbackup.bsock.ProtocolVersions.last, nqrustbackup.bsock.ProtocolVersions.nqrustbackup_12_4, nqrustbackup.bsock.ProtocolVersions.nqrustbackup_18_2):
              Specify the NQRustBackup Console protocol version to use.

           pam_username (str): Additional username when using PAM.

           pam_password (str): Additional user password when using PAM.

           tls_psk_enable (boolean): Enable TLS-PSK.

           tls_psk_require (boolean): Enforce using TLS-PSK.

           tls_version (None, ssl.PROTOCOL_TLS, ssl.PROTOCOL_TLSv1, ssl.PROTOCOL_TLSv1_1, ssl.PROTOCOL_TLSv1_2):
              TLS protocol version to use.

        Raises:
          nqrustbackup.exceptions.ConnectionError: On connections errors.
        """

        super(DirectorConsole, self).__init__()
        self.pam_username = pam_username
        self.pam_password = pam_password
        self.tls_psk_enable = tls_psk_enable
        self.tls_psk_require = tls_psk_require
        self.timeout = timeout
        if tls_version is not None:
            self.tls_version = tls_version
        self.identity_prefix = "R_CONSOLE"
        if protocolversion is not None and protocolversion > 0:
            self.requested_protocol_version = int(protocolversion)
            self.protocol_messages.set_version(self.requested_protocol_version)
        self.connect(
            address, port, dirname, ConnectionType.DIRECTOR, name, password, timeout
        )
        self._init_connection()
        self.max_reconnects = 1

    def _finalize_authentication(self):
        code, text = self.receive_and_evaluate_response_message()

        self.logger.debug("code: {0}".format(code))

        #
        # Test if PAM is requested.
        # If yes, handle PAM messages.
        #
        # Is is only available, with protocolversion >= ProtocolVersions.nqrustbackup_18_2,
        # however as it is optional,
        # it will be evaluated with all protocol versions.
        #
        if code == ProtocolMessageIds.PamRequired:
            self.logger.debug("PAM request: {0}".format(text))
            if (not self.pam_username) or (not self.pam_password):
                raise nqrustbackup.exceptions.PamAuthenticationError(
                    "PAM authentication is requested, but no PAM credentials given. Giving up.\n"
                )
            self.send(
                ProtocolMessages.pam_user_credentials(
                    self.pam_username, self.pam_password
                )
            )
            try:
                code, text = self.receive_and_evaluate_response_message()
            except nqrustbackup.exceptions.ConnectionLostError as e:
                raise nqrustbackup.exceptions.PamAuthenticationError(
                    "PAM authentication failed."
                )
        else:
            if (self.pam_username) or (self.pam_password):
                raise nqrustbackup.exceptions.PamAuthenticationError(
                    "PAM credentials provided, but this Director console does not offer PAM login. Giving up.\n"
                )

        #
        # Test if authentication has been accepted.
        #
        if code == ProtocolMessageIds.Ok:
            self.logger.info("Authentication: {0}".format(text))
            self.auth_credentials_valid = True
        else:
            raise nqrustbackup.exceptions.AuthenticationError(
                "Received unexcepted message: {0} {1} (expecting auth ok)".format(
                    code, text
                )
            )

        if self.get_protocol_version() >= ProtocolVersions.nqrustbackup_18_2:
            #
            # Handle info message.
            #
            code, text = self.receive_and_evaluate_response_message()
            if code == ProtocolMessageIds.InfoMessage:
                self.logger.debug("Info: {0}".format(text))
            else:
                raise nqrustbackup.exceptions.AuthenticationError(
                    "Received unexcepted message: {0} {1} (expecting info message)".format(
                        code, text
                    )
                )

    def _init_connection(self):
        self.call("autodisplay off")

    def _get_to_prompt(self):
        self.send(b".")
        return super(DirectorConsole, self)._get_to_prompt()
