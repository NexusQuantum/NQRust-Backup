<?php

/**
 *
 * nqrustbackup-webui - NQRust Backup Web Console
 *
 * @link      https://github.com/nqrustbackup/nqrustbackup for the canonical source repository
 * @copyright Copyright (C) 2013-2025 NQRustBackup GmbH & Co. KG (http://www.nqrustbackup.org/)
 * @license   GNU Affero General Public License (http://www.gnu.org/licenses/)
 * @author    Frank Bergkemper
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

namespace Application\View\Helper;

use Laminas\View\Helper\AbstractHelper;

class UpdateAlert extends AbstractHelper
{
    public function __invoke($version_info = null)
    {
        $result = null;

        if (!$version_info) {
            $result = '<span data-toggle="tooltip" data-placement="bottom" style="cursor: help;" title="Update information could not be retrieved"><span class="glyphicon glyphicon-exclamation-sign text-danger" aria-hidden="true"></span></span>';
        } elseif ($version_info['status'] != "uptodate") {
            $message = "NQRust Backup Director (" . $version_info["requested_version"] . "): " . $version_info['package_update_info'];
            $result = '<span data-toggle="tooltip" data-placement="bottom" style="cursor: help;" title="' . $message . '"><span class="glyphicon glyphicon-exclamation-sign text-danger" aria-hidden="true"></span></span>';
        }
        return $result;
    }
}
